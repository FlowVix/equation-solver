import * as wasm from "../wasm-lib/pkg/wasm_lib";

export class Equation {
    public id: number;
    public err_left: boolean = false;
    public err_right: boolean = false;

    constructor(public left: string, public right: string) {
        this.id = Math.random();
    }
}

export const DEFAULT_EQS: Equation[] = [
    new Equation("a * (b + 4)", "14"),
    new Equation("a + b", "5"),
];

const VAR_REGEX = /\b[A-Za-z_][A-Za-z0-9_']*(?!\s*\()\b/g;

export const detectVars = (eqs: string[]) => {
    let varSet = new Set<string>();
    for (let eq of eqs) {
        for (let match of eq.matchAll(VAR_REGEX)) {
            let v = match[0];
            if (!wasm.is_not_var(v)) {
                varSet.add(match[0]);
            }
        }
    }

    return [...varSet];
};

export const formatComplex = (re: number, im: number) => {
    re = parseFloat(re.toFixed(6));
    im = parseFloat(im.toFixed(6));

    if (im == 0) {
        return `${re}`;
    }
    if (im > 0) {
        return `${re} + ${im}i`;
    }
    return `${re} - ${-im}i`;
};
