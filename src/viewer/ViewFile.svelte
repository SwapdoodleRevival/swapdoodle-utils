<script lang="ts">
    import {
        BackendBPK1File,
        downloadBPK1Block,
        OpenedFile,
    } from "../lib/libdoodle/libdoodle.svelte";
    import type { SvelteComponent } from "svelte";
    import Unknown from "./blocks/Unknown.svelte";
    import { askForFile } from "../lib/files.svelte";
    import Icon from "@jamescoyle/svelte-icon";
    import { mdiPlus, mdiDownload, mdiTrashCan, mdiClose } from "@mdi/js";
    import { pushConfirm, pushPrompt } from "../lib/dialog.svelte";
    import DropTarget from "../components/DropTarget.svelte";
    import HexView from "../components/HexView.svelte";
    import { flip } from "svelte/animate";

    const READERS: { [key: string]: { default: () => SvelteComponent } } =
        import.meta.glob(["./blocks/*.svelte", "!./blocks/Unknown.svelte"], {
            eager: true,
        });

    let dragIndex: number | undefined = $state();

    let {
        file,
        onclose,
    }: {
        file: OpenedFile;
        onclose: () => any;
    } = $props();

    async function insertBlock() {
        let files = null;
        try {
            files = await askForFile();
        } catch {
            // files is set to null
        }
        let selected = files?.[0];

        if (selected) {
            let string = await pushPrompt("Enter block name", "");
            if (string) {
                file.addBlock(
                    string,
                    new Uint8Array(await selected.arrayBuffer()),
                );
            }
        }
    }

    async function close() {
        let result = await pushConfirm(
            "Closing file",
            "Do you really want to close this file?\nAll unsaved changes will be lost.",
        );
        if (result) {
            onclose();
        }
    }

    function buttonClass(active: boolean) {
        return (
            "btn px-3 py-2 text-start transition flex gap-2 " +
            (active
                ? "bg-yellow-400 hover:bg-yellow-500"
                : "hover:bg-yellow-300")
        );
    }

    function reorderFile(i: number) {
        if (dragIndex === undefined) {
            return;
        }
        if (i === dragIndex) {
            return;
        }
        file.reorderFile(dragIndex, i);
    }
</script>

{#snippet header(title: string, subtitle: string | null = null)}
    <div class="p-3 bg-yellow-200 border-b-2 border-b-yellow-700">
        <div class="font-bold">
            {title}
        </div>
        {#if subtitle}
            <div class="text-xs">
                {subtitle}
            </div>
        {/if}
    </div>
{/snippet}

<div class="flex grow overflow-y-hidden">
    <div
        class="md:w-70 w-30 flex flex-col shrink-0 shadow-xl bg-yellow-100 overflow-y-auto overflow-x-hidden"
    >
        {@render header("File options")}

        <button class={buttonClass(false)} onclick={() => file.download()}>
            <Icon path={mdiDownload} type="mdi" color="black"></Icon>
            Save BPK1 (uncompressed)
        </button>
        <button
            class={buttonClass(false)}
            onclick={() => file.downloadCompressed()}
        >
            <Icon path={mdiDownload} type="mdi" color="black"></Icon>
            Save BPK1 (compressed)
        </button>
        <button class={buttonClass(false)} onclick={close}>
            <Icon path={mdiClose} type="mdi" color="black"></Icon>
            Close file
        </button>

        {@render header("BPK1 Blocks", "Drag to change order.")}
        {#each file.blocks as block, i (block)}
            <div animate:flip={{ duration: 700 }}>
                <DropTarget
                    ondrop={() => {
                        reorderFile(i);
                    }}
                >
                    <div
                        draggable="true"
                        ondragstart={(e) => {
                            dragIndex = i;
                        }}
                        role="listitem"
                    >
                        <button
                            class="{buttonClass(
                                file.selectedBlock?.is_equal(block) ?? false,
                            )} w-full"
                            onclick={() => {
                                file.selectedBlock = block;
                            }}
                        >
                            {block.name}
                        </button>
                    </div>
                </DropTarget>
            </div>
        {/each}

        <button class={buttonClass(false)} onclick={insertBlock}>
            <Icon path={mdiPlus} type="mdi" color="black"></Icon>
            Insert block
        </button>
    </div>
    <div class="grow p-4 overflow-y-auto">
        {#if file.selectedBlock}
            <div class="flex flex-wrap gap-2 mb-2">
                <button
                    class="btn std flex gap-2"
                    onclick={() => downloadBPK1Block(file.selectedBlock!)}
                >
                    <Icon path={mdiDownload} type="mdi" color="black"></Icon>
                    Save block
                </button>
                <button
                    class="btn std flex gap-2"
                    onclick={async () => {
                        if (
                            await pushConfirm("Delete block", "Are you sure?")
                        ) {
                            file.bpk1File.delete_block(file.selectedBlock!);
                            file.updateBlocks();
                        }
                    }}
                >
                    <Icon path={mdiTrashCan} type="mdi" color="black"></Icon>
                    Delete block
                </button>
            </div>

            {@const Reader =
                READERS[`./blocks/${file.selectedBlock.name}.svelte`]?.default}
            {#if Reader}
                <Reader {file} block={file.selectedBlock} />
            {:else}
                <Unknown />
            {/if}

            <HexView block={file.selectedBlock}></HexView>
        {/if}
    </div>
</div>
