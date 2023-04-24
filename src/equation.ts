export const DEFAULT_EQS: [number, string, string][] = [
    [Math.random(), "a * (b + 4)", "14"],
    [Math.random(), "a + b", "5"],
];

const VAR_REGEX = /[A-Za-z_][A-Za-z0-9_']*/g;

export const detectVars = (eqs: string[]) => {
    let varSet = new Set<string>();
    for (let eq of eqs) {
        for (let match of eq.matchAll(VAR_REGEX)) {
            // console.log(match[0]);
            varSet.add(match[0]);
        }
    }
    varSet.delete("e");
    varSet.delete("pi");
    varSet.delete("i");

    return [...varSet];
};
