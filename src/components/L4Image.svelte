<script lang="ts">
    import type { HTMLImgAttributes } from "svelte/elements";
    import { CanvasContextCreationError } from "../lib/utils";
    import BlobImage from "./BlobImage.svelte";
    import { parse_l4_data } from "../lib/libdoodle/libdoodle.svelte";

    let {
        baseWidth,
        baseHeight,
        src,
        ...rest
    }: {
        baseWidth: number;
        baseHeight: number;
        src: number[][];
    } & Omit<HTMLImgAttributes, "src"> = $props();

    let imageBlobAwait = $derived(parse_l4_data(src, baseWidth, baseHeight));
</script>

{#await imageBlobAwait then imageBlob}
    <BlobImage src={imageBlob} {...rest}></BlobImage>
{/await}
