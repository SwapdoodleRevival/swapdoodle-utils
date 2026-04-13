<script lang="ts">
    import Doodle from "../../components/Doodle.svelte";
    import type {
        BPK1Block,
        Color,
        Colors,
        OpenedFile,
        Sheet,
    } from "../../lib/libdoodle/libdoodle.svelte";
    import Card from "../../components/Card.svelte";

    let {
        file,
        block,
    }: {
        file: OpenedFile;
        block: BPK1Block;
    } = $props();

    let availableColors = $derived(
        file.bpk1File.get_blocks().filter((k) => k.name === "COLSLT1"),
    );

    let sheet: Sheet = $derived(block.parse_sheet());

    let backupColors: Colors = {
        colors: [
            {
                primary: { r: 255, g: 0, b: 0, a: 255 },
                id: 0,
                name: "",
            } as Color,
            {
                primary: { r: 255, g: 255, b: 0, a: 255 },
                id: 0,
                name: "",
            } as Color,
            {
                primary: { r: 0, g: 255, b: 255, a: 255 },
                id: 0,
                name: "",
            } as Color,
            {
                primary: { r: 255, g: 0, b: 255, a: 255 },
                id: 0,
                name: "",
            } as Color,
            {
                primary: { r: 0, g: 255, b: 0, a: 255 },
                id: 0,
                name: "",
            } as Color,
        ],
    };

    let colors: Colors | null = $derived(
        availableColors ? availableColors[0].parse_colors() : backupColors,
    );
</script>

<Card style="info" title="About SHEET1" class="mb-2">
    SHEET1 blocks contain doodle data - the position of each stroke, the pen
    used (see COLSLT1), normal/bold state, 2D/3D state, etc.
</Card>
{#if !availableColors.length}
    <Card style="warning" title="No colors found!" class="mb-2">
        The currently opened file does not contain a COLSLT1 block, so no colors
        can be displayed. A backup set of colors has been used instead.
    </Card>
{/if}

<Doodle {sheet} {colors} />
