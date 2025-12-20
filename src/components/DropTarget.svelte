<script lang="ts">
    import type { Snippet } from "svelte";

    let {
        ondrop,
        children,
    }: {
        ondrop: (pos: number) => any;
        children: Snippet;
    } = $props();

    let self: HTMLDivElement | undefined = $state();

    let pos: number = $state(0);

    let highlight: string = $derived.by(() => {
        if (pos === 1) {
            return "border-t-2 border-t-black";
        }
        if (pos === -1) {
            return "border-b-2 border-b-black";
        }
        return "";
    });

    function dragover(e: DragEvent) {
        const r = self?.getBoundingClientRect();
        const top = r?.top ?? -1;
        const bottom = r?.bottom ?? -1;
        if (e.clientY > top && e.clientY < bottom) {
            if (e.clientY - top > bottom - e.clientY) {
                pos = -1;
            } else {
                pos = 1;
            }
        }
    }

    function dragend() {
        pos = 0;
    }
</script>

<div
    bind:this={self}
    ondragover={dragover}
    ondragleave={dragend}
    ondrop={() => {
        ondrop(pos);
        dragend();
    }}
    class={highlight}
    role="tooltip"
    aria-roledescription="drop area for the BPK1 block"
>
    {@render children()}
</div>
