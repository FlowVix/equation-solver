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

const VAR_REGEX = /[A-Za-z_][A-Za-z0-9_']*/g;

export const detectVars = (eqs: string[]) => {
    let varSet = new Set<string>();
    for (let eq of eqs) {
        for (let match of eq.matchAll(VAR_REGEX)) {
            varSet.add(match[0]);
        }
    }
    varSet.delete("e");
    varSet.delete("pi");
    varSet.delete("i");

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
