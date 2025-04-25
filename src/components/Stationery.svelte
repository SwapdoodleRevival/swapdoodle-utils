<script lang="ts">
    import type { Stationery } from "../lib/parsing/parsing.svelte";

    interface Props {
        stationery: Stationery;
    }

    let { stationery }: Props = $props();

    let stationery2d: string = $state("");
    let stationery3d: string = $state("");
    let stationeryMask: string = $state("");

    $effect(() => {
        stationery2d = URL.createObjectURL(
            new Blob([stationery.background_2d], { type: "image/jpeg" }),
        );
        stationery3d = URL.createObjectURL(
            new Blob([stationery.background_3d], { type: "image/jpeg" }),
        );

        let canvas = new OffscreenCanvas(256, 256);
        let ctx = canvas.getContext("2d");
        if (!ctx)
            throw new Error(
                "Unable to render Stationery mask (get canvas context failed)",
            );

        let iData = new ImageData(256, 256);

        let pos = 0;
        for (let row of stationery.mask) {
            for (let color of row) {
                iData.data[pos + 3] = color * 17;
                pos += 4;
            }
        }

        ctx.putImageData(iData, 0, 0);

        let valid = true;
        canvas.convertToBlob().then((blob) => {
            // Prevent a rare memory leak: 
            // convertToBlob() -> teardown() -> then() -> stationeryMask is not revoked
            if (!valid) return;
            stationeryMask = URL.createObjectURL(blob);
        });

        const teardown = () => {
            valid = false;
            URL.revokeObjectURL(stationery2d);
            URL.revokeObjectURL(stationery3d);
            URL.revokeObjectURL(stationeryMask);
        };

        return teardown;
    });
</script>

<div
    class="stationery"
    style:--s2d={`url(${stationery2d})`}
    style:--s3d={`url(${stationery3d})`}
    style:--sm={`url(${stationeryMask})`}
>
    <div class="s2d"></div>
    <div class="s3d"></div>
</div>

<style>
    .stationery {
        position: relative;
        width: 250px;
        height: 230px;
    }

    .stationery > div {
        position: absolute;
        width: 250px;
        height: 230px;
    }

    .stationery .s2d {
        background-image: var(--s2d);
    }

    .stationery .s3d {
        background-image: var(--s3d);
        mask-image: var(--sm);
    }
</style>
