<script lang="ts">
    import type { Sheet, Colors, Stationery } from "../lib/parsing/parsing.svelte";
    import StationeryComponent from "./Stationery.svelte";

    let canvas: HTMLCanvasElement | undefined = $state();

    interface Props {
        sheet: Sheet;
        colors?: Colors;
        stationery?: Stationery;
    }

    let { sheet, colors, stationery }: Props = $props();

    function sleep(ms: number) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }

    function getColor(index: number) {
        let color = colors?.colors[index];
        if (!color) return "rgba(0,0,0,255)";
        return `rgb(${color.r} ${color.g} ${color.b})`;
    }

    async function draw() {
        let c = canvas?.getContext("2d");
        if (!c) return;

        c.clearRect(0, 0, 255, 255);
        c.lineCap = "round";

        let old = { x: -1, y: -1 };

        for (let stroke of sheet.strokes) {
            c.lineWidth = stroke.style_bold ? 5 : 2;
            c.strokeStyle = getColor(stroke.style_color);
            c.fillStyle = c.strokeStyle;

            c.beginPath();
            if (old.x != -1) {
                c.moveTo(old.x, old.y);
                c.lineTo(stroke.x, stroke.y);
                c.stroke();
            }
            c.arc(stroke.x, stroke.y, c.lineWidth / 2, 0, 360);
            c.fill();

            if (stroke.draw_line) {
                old.x = stroke.x;
                old.y = stroke.y;
            } else {
                old.x = -1;
            }

            // await sleep(8);
        }
    }

    $effect(() => {
        draw();
    });
</script>

<div class="doodle">
    {#if stationery}
        <StationeryComponent {stationery}></StationeryComponent>
    {/if}

    <canvas class="drawing" bind:this={canvas} width="250" height="230">
    </canvas>
</div>

<style>
    .doodle {
        width: 250px;
        height: 230px;
        position: relative;
    }

    .doodle > :global(*) {
        position: absolute;
    }

    .drawing {
        image-rendering: pixelated;
    }
</style>
