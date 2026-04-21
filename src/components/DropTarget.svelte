<script lang="ts">
    import type { Snippet } from "svelte";

    let {
        ondrop,
        children,
    }: {
        ondrop: () => any;
        children: Snippet;
    } = $props();

    let highlight = $state(false);

    function dragover() {
        highlight = true;
    }

    function dragend() {
        highlight = false;
    }
</script>

<div
    ondragover={dragover}
    ondragleave={dragend}
    ondrop={() => {
        dragend();
        ondrop();
    }}
    class={[highlight ? "bg-black/10" : null, "transition-colors"]}
    role="tooltip"
    aria-roledescription="drop area for the BPK1 block"
>
    <div class={[highlight ? "translate-x-2" : null, "transition-transform"]}>
        {@render children()}
    </div>
</div>
