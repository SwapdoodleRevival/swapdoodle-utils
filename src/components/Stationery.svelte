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
        stationery2d = URL.createObjectURL(stationery.background_2d);
        stationery3d = URL.createObjectURL(stationery.background_3d);
        stationeryMask = URL.createObjectURL(stationery.mask);

        const teardown = () => {
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
