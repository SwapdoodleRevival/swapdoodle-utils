<script lang="ts">
    import BlobImage from "../../components/BlobImage.svelte";
    import Card from "../../components/Card.svelte";
    import type { BPK1File } from "../../lib/libdoodle/libdoodle.svelte";
    import type { BPK1Block } from "../../lib/libdoodle/wasm/libdoodle_wasm";

    let {
        file,
        block,
    }: {
        file: BPK1File,
        block: BPK1Block;
    } = $props();

    let blob = $derived(new Blob([block.data as BlobPart]));
</script>

<Card style="info" title="About THUMB2" class="mb-2">
    THUMB2 blocks contain thumbnails to show in the gallery.
</Card>

<BlobImage class="thumbnail" src={blob} style="height:200px" />
