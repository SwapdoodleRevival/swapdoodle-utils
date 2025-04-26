<script lang="ts">
    import type { HTMLImgAttributes } from "svelte/elements";

    let {
        src,
        ...restProps
    }: {
        src: Blob;
    } & Omit<HTMLImgAttributes, "src"> = $props();

    let url = $state("");

    $effect(() => {
        url = URL.createObjectURL(src);
        const teardown = () => {
            URL.revokeObjectURL(url);
        };
        return teardown;
    });
</script>

<img src={url} {...restProps} />
