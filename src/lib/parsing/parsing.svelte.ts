import toast from "../toast.svelte";
import loadWasm, {
    init_error_handler,
    decompress,
    parse_letter,
    decompress_if_compressed,
    type JsLetter,
    type Sheet,
    type SheetStroke,
    type Colors,
    type JsStationery
} from "./wasm/parsing_wasm";

async function init() {
    await loadWasm();
    init_error_handler();
}

type Modify<T, R> = Omit<T, keyof R> & R;

type Letter = Modify<JsLetter, {
    stationery?: Stationery,
    thumbnails: Blob[]
}>;

type Stationery = Modify<JsStationery, {
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
            throw new Error(
                "Unable to render Stationery mask (get canvas context failed)",
            );
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

class LetterFile {
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
        LetterFile.downloadFile(decompress_if_compressed(this.letterData), fileName);
    }

    public downloadBpkBlock(block: string, index: number) {
        let blockData = this.letter.blocks.get(block)?.[index];
        if (!blockData)
            throw new Error("Nonexistent block");
        LetterFile.downloadFile(blockData, `${block}$${index}.bin`)
    }

    private static downloadFile(data: Uint8Array, as: string) {
        let blob = new Blob([data], {
            type: "application/octet-stream",
        });
        let downloadUrl = URL.createObjectURL(blob);
        let a = document.createElement("a");
        a.download = as;
        a.href = downloadUrl;
        a.click();
        URL.revokeObjectURL(downloadUrl);
    }

    /*   async function prepCanvas() {
           let part2d = new OffscreenCanvas(250, 230);
           let part3d = new OffscreenCanvas(256, 256);
           let part2dCtx = part2d.getContext("2d")!;
           let part3dCtx = part3d.getContext("2d")!;
           part2dCtx.drawImage(
               await createImageBitmap(
                   new Blob([stationery.background_2d], { type: "image/jpeg" }),
               ),
               0,
               0,
           );
           part3dCtx.drawImage(
               await createImageBitmap(
                   new Blob([stationery.background_3d], { type: "image/jpeg" }),
               ),
               0,
               0,
           );
           let imgData = part3dCtx.getImageData(0, 0, 256, 256);
           let pos = 0;
           for (let row of stationery.mask) {
               for (let color of row) {
                   imgData.data[pos + 3] = color * 17;
                   pos += 4;
               }
           }
           part3dCtx.putImageData(imgData, 0, 0);
           part2dCtx.drawImage(part3d, 0, 0);
           return await part2d.convertToBlob();
       }*/
}

export { init, LetterFile, decompress, decompress_if_compressed, type Letter, type Sheet, type SheetStroke, type Colors, type Stationery };
