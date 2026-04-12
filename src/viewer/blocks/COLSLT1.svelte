<script lang="ts">
    import Card from "../../components/Card.svelte";
    import type {
        BPK1Block,
        BPK1File,
        Colors,
        RGBA,
    } from "../../lib/libdoodle/libdoodle.svelte";

    let {
        file,
        block,
    }: {
        file: BPK1File;
        block: BPK1Block;
    } = $props();

    let colors: Colors = $derived(block.parse_colors());

    function getCSSFromRGBA(rgba: RGBA) {
        return `rgba(${rgba.r},${rgba.g},${rgba.b},${rgba.a})`;
    }
</script>

<Card style="info" title="About COLSLT1" class="mb-2">
    COLSLT1 blocks contain the pens used in the doodles (see SHEET1). This
    includes the name and RGB color.
</Card>

{#each colors.colors as color}
    <div class="flex items-center gap-2 mb-2">
        <div
            class="w-8 h-8 rounded-full overflow-hidden relative"
            style="background-color: {getCSSFromRGBA(color.primary)}"
        >
            <div
                class="w-[50%] h-[50%] absolute bottom-0"
                style="background-color: {getCSSFromRGBA(color.extra1)}"
            ></div>
            <div
                class="w-[50%] h-[50%] absolute right-0 top-0"
                style="background-color: {getCSSFromRGBA(color.extra2)}"
            ></div>
            <div
                class="w-[50%] h-[50%] absolute right-0 bottom-0"
                style="background-color: {getCSSFromRGBA(color.extra3)}"
            ></div>
        </div>

        <div class="flex flex-col">
            <span class="font-bold">
                {color.name}
            </span>
            ID: {color.id}
        </div>
    </div>
{/each}
