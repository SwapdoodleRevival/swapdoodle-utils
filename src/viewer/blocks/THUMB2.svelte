<script lang="ts">
    import BlobImage from "../../components/BlobImage.svelte";
    import Card from "../../components/Card.svelte";
    import type { OpenedFile } from "../../lib/libdoodle/libdoodle.svelte";
    import type { BackendBPK1Block } from "../../lib/libdoodle/wasm/libdoodle_wasm";

    let {
        file,
        block,
    }: {
        file: OpenedFile;
        block: BackendBPK1Block;
    } = $props();

    let blob = $derived(new Blob([block.data as BlobPart]));
</script>

<Card style="info" title="About THUMB2" class="mb-2">
    THUMB2 blocks contain thumbnails to show in the gallery.
</Card>

<BlobImage class="thumbnail" src={blob} style="height:200px" />
