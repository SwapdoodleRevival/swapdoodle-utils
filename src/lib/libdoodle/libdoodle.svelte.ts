import { CanvasContextCreationError, invokeDownload } from "../utils";
import loadWasm, {
    init as initWasm,
    decompress,
    parse_letter,
    decompress_if_compressed,
    type JsLetter,
    type Sheet,
    type SheetStroke,
    type Colors,
    type JsStationery
} from "./wasm/libdoodle_wasm";

async function init() {
    await loadWasm();
    initWasm();
}

await init();

type Modify<T, R> = Omit<T, keyof R> & R;

export type Letter = Modify<JsLetter, {
    stationery?: Stationery,
    thumbnails: Blob[]
}>;

export type Stationery = Modify<JsStationery, {
    background_2d: Blob
    background_3d: Blob
    mask: Blob
}>;

async function postProcessLetter(jsLetter: JsLetter): Promise<Letter> {
    // @ts-ignore
    let letter: Letter = jsLetter

    if (jsLetter.stationery) {
        letter.stationery!.background_2d = new Blob([jsLetter.stationery.background_2d], { type: "image/jpeg" })
        letter.stationery!.background_3d = new Blob([jsLetter.stationery.background_3d], { type: "image/jpeg" })

        let canvas = new OffscreenCanvas(256, 256);
        let ctx = canvas.getContext("2d");
        if (!ctx)
            throw new CanvasContextCreationError();
        let imgData = new ImageData(256, 256);

        let pos = 0;
        for (let row of jsLetter.stationery.mask) {
            for (let color of row) {
                imgData.data[pos + 3] = color * 17;
                pos += 4;
            }
        }

        ctx.putImageData(imgData, 0, 0);

        letter.stationery!.mask = await canvas.convertToBlob();
    }

    letter.thumbnails = jsLetter.thumbnails.map(data => new Blob([data], { type: "image/jpeg" }))

    return letter;
}

export class LetterFile {
    public letter: Letter = $state()!
    public letterData!: Uint8Array<ArrayBuffer>;

    // disable direct construction - use readFile
    private constructor() { };

    public static readFile(file: File): Promise<LetterFile> {
        return new Promise((resolve, reject) => {
            let reader = new FileReader();

            reader.onload = (readerEvent) => {
                let content = readerEvent.target?.result as ArrayBuffer | null;
                if (!content) {
                    reject(Error("Could not read file content"));
                    return;
                }

                let letterData = new Uint8Array(content);

                try {
                    let letter = parse_letter(letterData);
                    let file = new LetterFile();
                    postProcessLetter(letter).then(
                        letter => {
                            file.letter = letter;
                            resolve(file);
                        }
                    )
                    file.letterData = letterData;
                    return;
                } catch {
                    // TODO: Smarter errors from Rust
                    reject(Error("This file does not seem to be a Swapdoodle Letter."))
                }
            };

            reader.readAsArrayBuffer(file);
        })
    }

    public downloadDecompressedBpk(fileName: string) {
        invokeDownload(decompress_if_compressed(this.letterData), fileName);
    }

    public downloadBpkBlock(block: string, index: number) {
        let blockData = this.letter.blocks.get(block)?.[index];
        if (!blockData)
            throw new Error("Nonexistent block");
        invokeDownload(blockData, `${block}$${index}.bin`)
    }

    public async flattenStationery() {
        let stationery = new OffscreenCanvas(250, 230);
        if (this.letter.stationery) {
            let ctx2d = stationery.getContext("2d")!;

            let part3d = new OffscreenCanvas(256, 256);
            let partMask = new OffscreenCanvas(256, 256);

            let ctx3d = part3d.getContext("2d")!;
            let ctxMask = partMask.getContext("2d")!;

            ctx2d.drawImage(await createImageBitmap(this.letter.stationery.background_2d), 0, 0);
            ctx3d.drawImage(await createImageBitmap(this.letter.stationery.background_3d), 0, 0);
            ctxMask.drawImage(await createImageBitmap(this.letter.stationery.mask), 0, 0);

            let imgData = ctx3d.getImageData(0, 0, 256, 256);
            let maskData = ctxMask.getImageData(0, 0, 256, 256);
            for (let pos = 3; pos < 256 * 256 * 4; pos += 4) {
                imgData.data[pos] = maskData.data[pos];
            }
            ctx3d.putImageData(imgData, 0, 0);
            ctx2d.drawImage(part3d, 0, 0);
        }
        return stationery;
    }
}

export { decompress, decompress_if_compressed, type Sheet, type SheetStroke, type Colors };
