<script lang="ts">
    import BlobImage from "../../components/BlobImage.svelte";
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

<BlobImage class="thumbnail" src={blob} height="200px" />
