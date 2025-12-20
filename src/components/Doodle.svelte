<script lang="ts">
    import type { Sheet, Colors } from "../lib/libdoodle/libdoodle.svelte";
    import { CanvasContextCreationError } from "../lib/utils";

    interface Props {
        sheet: Sheet;
        colors?: Colors;
        show2d?: boolean;
        canvas3d?: HTMLCanvasElement | undefined;
        show3d?: boolean;
        canvas2d?: HTMLCanvasElement | undefined;
    }

    let {
        sheet,
        colors,
        show2d = true,
        canvas3d = $bindable(),
        show3d = true,
        canvas2d = $bindable(),
    }: Props = $props();

    function sleep(ms: number) {
        return new Promise((resolve) => setTimeout(resolve, ms));
    }

    function getColor(index: number) {
        let color = colors?.colors[index];
        if (!color) return "rgba(0,0,0,255)";
        return `rgb(${color.primary.r} ${color.primary.g} ${color.primary.b})`;
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
</script>

<div class="doodle">
    <canvas
        style:display={show2d ? "inherit" : "none"}
        class="drawing"
        bind:this={canvas2d}
        width="250"
        height="230"
    >
    </canvas>
    <canvas
        style:display={show3d ? "inherit" : "none"}
        class="drawing"
        bind:this={canvas3d}
        width="250"
        height="230"
    >
    </canvas>
</div>

<style>
    .doodle {
        position: relative;
    }

    .doodle,
    .drawing {
        left: 0;
        top: 0;
        width: 250px;
        height: 230px;
    }
    
    .drawing {
        position: absolute;
        image-rendering: pixelated;
    }
</style>
