<script lang="ts">
    import { BPK1File } from "../lib/libdoodle/libdoodle.svelte";
    import type { SvelteComponent } from "svelte";
    import Unknown from "./blocks/Unknown.svelte";
    import { askForFile } from "../lib/files.svelte";
    import Icon from "@jamescoyle/svelte-icon";
    import { mdiPlus, mdiDownload, mdiTrashCan, mdiClose } from "@mdi/js";
    import { pushDialog } from "../lib/dialog.svelte";
    import DropTarget from "../components/DropTarget.svelte";
    import HexView from "../components/HexView.svelte";

    const READERS: { [key: string]: { default: () => SvelteComponent } } =
        import.meta.glob(["./blocks/*.svelte", "!./blocks/Unknown.svelte"], {
            eager: true,
        });

    let dragIndex: number | undefined = $state();

    let {
        file,
        onclose,
    }: {
        file: BPK1File;
        onclose: () => any;
    } = $props();

    async function insertBlock() {
        let files = await askForFile();
        let selected = files?.[0];

        if (selected) {
            let string = prompt("Block name? (max 7 characters)");
            if (string) {
                let name = string.substring(0, 7);

                file.blocks.push({
                    name: name,
                    data: new Uint8Array(await selected.arrayBuffer()),
                });
            }
        }
    }

    async function close() {
        let result = await pushDialog({
            title: "Closing file",
            message:
                "Do you really want to close this file?\nAll unsaved changes will be lost.",
            buttons: [
                { id: "yes", label: "Yes" },
                { id: "no", label: "No" },
            ],
        });
        if (result === "yes") {
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

    function reorderFile(i: number, pos: number) {
        if (dragIndex === undefined) {
            return;
        }
        if (i === dragIndex) {
            return;
        }
        let target = file.blocks[i];
        let move = file.blocks.splice(dragIndex, 1)[0];
        i = file.blocks.indexOf(target);
        i += pos === 1 ? 0 : 1;
        if (i < 0) {
            i = 0;
        } else if (i >= file.blocks.length) {
            i = file.blocks.length;
        }
        file.blocks.splice(i, 0, move);
    }
</script>

{#snippet header(label: string)}
    <div class="p-3 bg-yellow-200 border-b-2 border-b-yellow-700 font-bold">
        {label}
    </div>
{/snippet}

<div class="flex grow">
    <div class="md:w-70 w-30 flex flex-col shrink-0 shadow-xl bg-yellow-100">
        {@render header("File options")}

        <button
            class={buttonClass(false)}
            onclick={() => file.downloadDecompressedBpk()}
        >
            <Icon path={mdiDownload} type="mdi" color="black"></Icon>
            Save BPK1 (uncompressed)
        </button>
        <button
            class={buttonClass(false)}
            onclick={() => file.downloadCompressedBpk()}
        >
            <Icon path={mdiDownload} type="mdi" color="black"></Icon>
            Save BPK1 (compressed)
        </button>
        <button class={buttonClass(false)} onclick={close}>
            <Icon path={mdiClose} type="mdi" color="black"></Icon>
            Close file
        </button>

        {@render header("BPK1 Blocks")}
        {#each file.blocks as block, i (block)}
            <DropTarget
                ondrop={(pos) => {
                    reorderFile(i, pos);
                }}
            >
                <div
                    draggable="true"
                    ondragstart={(e) => {
                        dragIndex = i;
                        e.dataTransfer?.setData(
                            "application/octet-stream",
                            "asdf",
                        );
                    }}
                    role="listitem"
                >
                    <button
                        class="{buttonClass(
                            file.selectedBlock === block,
                        )} w-full"
                        onclick={() => file.selectBlock(i)}
                    >
                        {block.name}
                    </button>
                </div>
            </DropTarget>
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
                    onclick={() => file.downloadBpkBlock(file.selectedBlock!)}
                >
                    <Icon path={mdiDownload} type="mdi" color="black"></Icon>
                    Save block
                </button>
                <button
                    class="btn std flex gap-2"
                    onclick={() => file.deleteSelectedBlock()}
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
