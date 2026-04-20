<script lang="ts">
    import BlobImage from "../../components/BlobImage.svelte";
    import type { OpenedFile } from "../../lib/libdoodle/libdoodle.svelte";
    import type { BackendBPK1Block } from "../../lib/libdoodle/wasm/libdoodle_wasm";

    let {
        file,
        block,
    }: {
        file: OpenedFile,
        block: BackendBPK1Block;
    } = $props();

    let blob = $derived(new Blob([block.data as BlobPart]));
</script>

<BlobImage class="thumbnail" src={blob} height="200px" />
