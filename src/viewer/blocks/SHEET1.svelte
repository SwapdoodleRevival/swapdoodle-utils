<script lang="ts">
    import Doodle from "../../components/Doodle.svelte";
    import type {
        BPK1Block,
        BPK1File,
        Color,
        Colors,
        Sheet,
    } from "../../lib/libdoodle/libdoodle.svelte";
    import {
        parse_colors,
        parse_sheet,
    } from "../../lib/libdoodle/libdoodle.svelte";
    import Card from "../../components/Card.svelte";
    import { onMount } from "svelte";

    let {
        file,
        block,
    }: {
        file: BPK1File;
        block: BPK1Block;
    } = $props();

    let availableColors = $derived(
        file.blocks.filter((k) => k.name === "COLSLT1"),
    );

    let sheet: Sheet = $derived(parse_sheet(block));

    let selectedColorsBlock: BPK1Block | null = $state(null);

    let backupColors: Colors = {
        colors: [
            // ignore extra colors for now
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

    onMount(() => {
        selectedColorsBlock = availableColors ? availableColors[0] : null;
    });

    let colors: Colors | null = $derived(
        selectedColorsBlock ? parse_colors(selectedColorsBlock) : backupColors,
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
