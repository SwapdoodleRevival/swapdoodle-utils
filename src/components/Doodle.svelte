<script lang="ts">
    import type {
        Sheet,
        Colors,
        Stationery,
        LetterFile,
    } from "../lib/libdoodle/libdoodle.svelte";
    import { info } from "../lib/toast.svelte";
    import {
        CanvasContextCreationError,
        invokeDownload,
        LoudError,
    } from "../lib/utils";
    import StationeryComponent from "./Stationery.svelte";

    let canvas3d: HTMLCanvasElement | undefined = $state();
    let canvas2d: HTMLCanvasElement | undefined = $state();

    let displayOptions = $state({
        scale: 1.5,
        show2d: true,
        show3d: true,
        showStationery: true,
    });

    interface Props {
        file: LetterFile;
        sheet: Sheet;
        colors?: Colors;
        stationery?: Stationery;
    }

    let { file, sheet, colors, stationery }: Props = $props();

    function sleep(ms: number) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }

    function getColor(index: number) {
        let color = colors?.colors[index];
        if (!color) return "rgba(0,0,0,255)";
        return `rgb(${color.r} ${color.g} ${color.b})`;
    }

    async function draw() {
        let c2d = canvas2d?.getContext("2d");
        let c3d = canvas3d?.getContext("2d");
        if (!c2d || !c3d) throw new CanvasContextCreationError();

        let pen: Partial<CanvasPathDrawingStyles & CanvasFillStrokeStyles> = {
            lineCap: "round",
            lineWidth: 2,
            fillStyle: "",
            strokeStyle: "",
        };

        c2d.clearRect(0, 0, 255, 255);
        c3d.clearRect(0, 0, 255, 255);
        c2d.lineCap = c3d.lineCap = "round";

        let old = { x: -1, y: -1 };

        for (let stroke of sheet.strokes) {
            pen.lineWidth = stroke.style_bold ? 5 : 2;
            pen.fillStyle = pen.strokeStyle = getColor(stroke.style_color);

            let c = stroke.style_3d ? c3d : c2d;
            Object.assign(c, pen);

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

    async function exportDoodle() {
        if (
            !displayOptions.show2d &&
            !displayOptions.show3d &&
            !displayOptions.showStationery
        ) {
            info({
                title: "What?",
                message:
                    "You just tried to export a blank image. Why would you do that?\nEnable some of the layers first and then try again.",
            });
            return;
        }

        let result = new OffscreenCanvas(250, 230);
        let resultCtx = result.getContext("2d");
        if (!resultCtx) throw new CanvasContextCreationError();

        if (displayOptions.showStationery) {
            resultCtx.drawImage(await file.flattenStationery(), 0, 0);
        }

        if (displayOptions.show2d) {
            if (!canvas2d) {
                throw new Error("Canvas layer for 2D is not initialized yet");
            }
            resultCtx.drawImage(canvas2d, 0, 0);
        }

        if (displayOptions.show2d) {
            if (!canvas3d) {
                throw new Error("Canvas layer for 3D is not initialized yet");
            }
            resultCtx.drawImage(canvas3d, 0, 0);
        }

        invokeDownload(await result.convertToBlob(), "doodle.png");
    }
</script>

<div class="doodle-wrapper" style:--scale={displayOptions.scale}>
    <div class="doodle">
        {#if stationery}
            <div
                style:display={displayOptions.showStationery
                    ? "inherit"
                    : "none"}
                class="stationery"
            >
                <StationeryComponent {stationery}></StationeryComponent>
            </div>
        {/if}
        <canvas
            style:display={displayOptions.show2d ? "inherit" : "none"}
            class="drawing"
            bind:this={canvas2d}
            width="250"
            height="230"
        >
        </canvas>
        <canvas
            style:display={displayOptions.show3d ? "inherit" : "none"}
            class="drawing"
            bind:this={canvas3d}
            width="250"
            height="230"
        >
        </canvas>
    </div>
    <div class="controls">
        <div class="controls-row">
            <span>
                <b>2D</b>
                <input type="checkbox" bind:checked={displayOptions.show2d} />
            </span>
            <span>
                <b>3D</b>
                <input type="checkbox" bind:checked={displayOptions.show3d} />
            </span>
            <span>
                <b>Background</b>
                <input
                    type="checkbox"
                    bind:checked={displayOptions.showStationery}
                />
            </span>
        </div>
        <div class="controls-row centered">
            <button onclick={exportDoodle}>Save as image</button>
        </div>
    </div>
</div>

<style>
    .doodle-wrapper {
        box-shadow:
            0 3px 6px rgba(0, 0, 0, 0.16),
            0 3px 6px rgba(0, 0, 0, 0.23);
        background-color: rgba(255, 255, 255, 0.2);
    }

    .doodle {
        position: relative;
    }

    .doodle,
    .drawing {
        width: calc(250px * var(--scale));
        height: calc(230px * var(--scale));
    }

    .stationery {
        transform: scale(var(--scale));
        transform-origin: top left;
    }

    .doodle > :global(*) {
        position: absolute;
    }

    .drawing {
        image-rendering: pixelated;
    }

    .controls {
        padding: 8px;
        background-color: white;
    }

    .controls-row {
        display: flex;
        justify-content: space-between;
    }

    .centered {
        justify-content: center;
    }
</style>
