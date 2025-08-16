import { CanvasContextCreationError, invokeDownload } from "../utils";
import loadWasm, {
    init as initWasm,
    decompress,
    parse_bpk1,
    build_bpk1,
    decompress_if_compressed,
    type Sheet,
    type SheetStroke,
    type Colors,
    type BPK1Block,
    parse_stationery,
    compress_lz11,
} from "./wasm/libdoodle_wasm";
export * from "./wasm/libdoodle_wasm";

async function init() {
    await loadWasm();
    initWasm();
}

await init();

export class BPK1File {
    public fileName: string = $state("unnamed.bpk1");
    public blocks: BPK1Block[] = $state([])
    public selectedBlock: BPK1Block | null = $state.raw(null);

    // disable direct construction - use readFile
    private constructor() { };

    public static readFile(file: File): Promise<BPK1File> {
        return new Promise((resolve, reject) => {
            let reader = new FileReader();

            reader.onload = async (readerEvent) => {
                let content = readerEvent.target?.result as ArrayBuffer | null;
                if (!content) {
                    reject(Error("Could not read file content"));
                    return;
                }

                let letterData = new Uint8Array(content);
                try {
                    resolve(await this.readUint8Array(letterData));
                } catch (e) {
                    reject(e);
                }
            };

            reader.readAsArrayBuffer(file);
        })
    }

    public static async readUint8Array(letterData: Uint8Array<ArrayBufferLike>) {
        try {
            let file = new BPK1File();
            file.blocks = parse_bpk1(letterData);
            return file;
        } catch (e) {
            console.warn(e)
            // TODO: Smarter errors from Rust
            throw new Error("This file does not seem to be a Swapdoodle archive.")
        }
    }

    public downloadDecompressedBpk() {
        invokeDownload(build_bpk1(this.blocks), this.fileName);
    }

    public downloadCompressedBpk() {
        invokeDownload(compress_lz11(build_bpk1(this.blocks), 20000), this.fileName);
    }

    public downloadBpkBlock(block: BPK1Block) {
        invokeDownload(block.data, `${block.name}.bin`)
    }

    public selectBlock(block: number) {
        this.selectedBlock = this.blocks[block] ?? null;
    }

    public deleteSelectedBlock(): any {
        if (!this.selectedBlock) {
            return;
        }

        let block = this.blocks.indexOf(this.selectedBlock);
        this.blocks.splice(block, 1)
        if (this.blocks.length <= block) {
            block = this.blocks.length - 1;
        }
        this.selectedBlock = this.blocks[block];
    }
}

export async function parse_l4_data(src: number[][], width: number, height: number) {
    let canvas = new OffscreenCanvas(width, height);
    let ctx = canvas.getContext("2d");
    if (!ctx) throw new CanvasContextCreationError();
    let imgData = new ImageData(width, height);
    let pos = 0;
    for (let row of src) {
        for (let color of row) {
            imgData.data[pos + 3] = color * 17;
            pos += 4;
        }
    }
    ctx.putImageData(imgData, 0, 0);
    return await canvas.convertToBlob();
}

export async function parse_and_flatten_stationery(block: BPK1Block) {
    let result = new OffscreenCanvas(250, 230);
    let stationery = parse_stationery(block);
    let ctx2d = result.getContext("2d")!;
    let part3d = new OffscreenCanvas(256, 256);
    let partMask = new OffscreenCanvas(256, 256);
    let ctx3d = part3d.getContext("2d")!;
    let ctxMask = partMask.getContext("2d")!;
    ctx2d.drawImage(await createImageBitmap(new Blob([stationery.background_2d] as BlobPart[])), 0, 0);
    ctx3d.drawImage(await createImageBitmap(new Blob([stationery.background_3d] as BlobPart[])), 0, 0);
    ctxMask.drawImage(await createImageBitmap(await parse_l4_data(stationery.mask, 256, 256)), 0, 0);
    let imgData = ctx3d.getImageData(0, 0, 256, 256);
    let maskData = ctxMask.getImageData(0, 0, 256, 256);
    for (let pos = 3; pos < 256 * 256 * 4; pos += 4) {
        imgData.data[pos] = maskData.data[pos];
    }
    ctx3d.putImageData(imgData, 0, 0);
    ctx2d.drawImage(part3d, 0, 0);
    return result;
}

export { decompress, decompress_if_compressed, type Sheet, type SheetStroke, type Colors };
