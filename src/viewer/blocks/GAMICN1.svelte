<script lang="ts">
    import type {
        BPK1Block,
        OpenedFile,
    } from "../../lib/libdoodle/libdoodle.svelte";
    import { openNewFile } from "../../lib/files.svelte";
    import Card from "../../components/Card.svelte";

    let {
        file,
        block,
    }: {
        file: OpenedFile;
        block: BPK1Block;
    } = $props();
</script>

<Card style="info" title="About GAMICN1" class="mb-2">
    GAMICN1 blocks contain game icon data (as seen in stickers).

    GAMICN1 blocks themselves are BPK1 archives.
    <div class="mt-2">
        <button
            class="btn std"
            onclick={() => {
                openNewFile(block.data, `${file.fileName} - GAMICN1.bpk` );
            }}>Open in new tab</button
        >
    </div>
</Card>

<Card style="warning" title="Not supported yet" class="mb-2">
    Parsing is not supported yet!
</Card>