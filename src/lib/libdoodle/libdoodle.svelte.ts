import { CanvasContextCreationError, invokeDownload } from "../utils";
import loadWasm, {
    init as initWasm,
    BackendBPK1Block,
    BackendBPK1File
} from "./wasm/libdoodle_wasm";
export * from "./wasm/libdoodle_wasm";
export type * from "./wasm/libdoodle_wasm";

async function init() {
    await loadWasm();
    initWasm();
}

await init();

export class OpenedFile {
    public fileName: string = $state("unnamed.bpk1");
    public bpk1File: BackendBPK1File;
    public selectedBlock: BackendBPK1Block | null = $state.raw(null);
    private _blocks: BackendBPK1Block[] = $state([]);

    constructor() {
        this.bpk1File = new BackendBPK1File();
    }

    private static fromWrapped(name: string, file: BackendBPK1File) {
        let of = new OpenedFile();
        of.fileName = name;
        of.bpk1File = file;
        of.updateBlocks();
        return of;
    }

    public static readFile(file: File): Promise<OpenedFile> {
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
                    resolve(OpenedFile.fromBytes(file.name, letterData));
                } catch (e) {
                    reject(e);
                }
            };

            reader.readAsArrayBuffer(file);
        })
    }

    public static fromBytes(name: string, data: Uint8Array<ArrayBufferLike>) {
        try {
            return OpenedFile.fromWrapped(name, BackendBPK1File.from_bpk1_bytes(data));
        } catch (e) {
            console.warn(e)
            // TODO: Smarter errors from Rust
            throw new Error("This file does not seem to be a Swapdoodle archive.")
        }
    }

    public get blocks() {
        return this._blocks;
    }

    public updateBlocks() {
        this._blocks = this.bpk1File.get_blocks();
    }

    public download(new_filename: string | null = null) {
        invokeDownload(
            this.bpk1File.to_uncompressed_bpk1_archive(),
            new_filename ?? this.fileName
        )
    }

    public downloadCompressed(new_filename: string | null = null) {
        invokeDownload(
            this.bpk1File.to_lz11_bpk1_archive(20000),
            new_filename ?? this.fileName
        )
    }

    public reorderFile(i: number, pos: number) {
        console.log(`Reorder ${i} to ${pos}`);
        let target = this._blocks[i];
        this.bpk1File.reorder_block(target, pos);
        this.updateBlocks();
    }

    public addBlock(name: string, bytes: Uint8Array<ArrayBuffer>) {
        this.bpk1File.insert_bpk1_block(name, bytes);
        this.updateBlocks();
    }
}

export function downloadBPK1Block(block: BackendBPK1Block) {
    invokeDownload(block.data, `${block.name}.bin`);
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

export async function parse_and_flatten_stationery(block: BackendBPK1Block) {
    let result = new OffscreenCanvas(250, 230);
    let stationery = block.parse_stationery();
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
