import { useState, useEffect } from "react";
import "./App.css";

import * as wasm from "../wasm-lib/pkg/wasm_lib";
import { DEFAULT_EQS, Equation, detectVars, formatComplex } from "./equation";
import { PositionedError } from "../wasm-lib/pkg/wasm_lib";

const DESCRIPTIONS = [
    {
        text: [
            "This is a numerical solver for systems of equations.",
            "It works by using a version of the Newton-Raphson method\
            modified to work with matrices. An initial random guess for each\
            variable is picked, after which this iterative method is applied\
            until convergence.",
            "You can control the amount of attempts to find a solution, the number of iterations per attempt, and the epsilon for equality",
        ],
    },
    {
        title: "Variables",
        text: [
            "The solver will auto-detect variables from the equations you\
            input. The allowed naming scheme is any combination of\
            alphanumeric characters, underscores, and apostrophes, but must\
            not start with a number.",
        ],
    },
    {
        title: "Equations",
        text: [
            "Supported operators: + - * / ^ %",
            "Supported functions: sqrt, ln, all trig and hyperbolic trig functions and inverses",
            "Predefined constants: e, pi, i",
            "Complex numbers are supported",
        ],
    },
];

const App = () => {
    let [equations, setEquations] = useState([...DEFAULT_EQS]);
    const flattenEqs = () => equations.flatMap(eq => [eq.left, eq.right]);

    let [vars, setVars] = useState(detectVars(flattenEqs()));

    useEffect(() => {
        setVars(detectVars(flattenEqs()));
    }, [equations]);

    let [errMsg, setErrMsg] = useState("");
    const EMPTY_SOLUTION: [string, [number, number]][] = [];
    let [solution, setSolution] = useState(EMPTY_SOLUTION);

    const solve = () => {
        setErrMsg("");
        setSolution([]);
        for (let eq of equations) {
            eq.err_left = false;
            eq.err_right = false;
        }

        try {
            let initial = vars.map(_ => Math.random() * 100 - 50);
            let solution = wasm.solve(
                equations.map(eq => [eq.left, eq.right]),
                1000,
                new Float64Array(initial)
            );
            if (solution != undefined) {
                setSolution(solution);
            } else {
                setErrMsg("No solutions found");
            }
        } catch (e) {
            if (e instanceof PositionedError) {
                setErrMsg(e.msg);
                if (e.second) {
                    equations[e.eq].err_right = true;
                } else {
                    equations[e.eq].err_left = true;
                }
            }
        }
        setEquations([...equations]);
    };

    return (
        <div className="everything">
            <div className="main_panel">
                <h1>Equation Solver</h1>

                {DESCRIPTIONS.map((data, i) => {
                    let content = data.text.map((t, i) => (
                        <div key={i}>
                            {t}
                            <br />
                        </div>
                    ));

                    return (
                        <div key={i}>
                            <br />
                            {data.title ? (
                                <>
                                    <h3>{data.title}</h3>
                                    <hr />
                                </>
                            ) : (
                                <></>
                            )}
                            {content}
                        </div>
                    );
                })}
                <br />
                <h3>Solver</h3>
                <hr />
                <div className="solver">
                    <span>
                        <button
                            className="add_button"
                            onClick={() => {
                                setEquations([
                                    ...equations,
                                    new Equation("", ""),
                                ]);
                            }}
                        >
                            Add equation
                        </button>
                        Detected variables: {vars.join(", ")}
                    </span>
                    {errMsg.length != 0 ? (
                        <>
                            <br />
                            <span className="errormsg">{errMsg}</span>
                        </>
                    ) : (
                        <></>
                    )}
                    <br />

                    {equations.map((eq, i) => (
                        <div className="equation" key={eq.id}>
                            <button
                                className="remove_button"
                                onClick={() => {
                                    equations.splice(i, 1);
                                    setEquations([...equations]);
                                    // setVars(detectVars(flattenEqs()));
                                }}
                            >
                                <span className="material-symbols-outlined">
                                    delete
                                </span>
                            </button>
                            <input
                                type="text"
                                className={`equation_input ${
                                    eq.err_left ? "err" : ""
                                }`}
                                defaultValue={eq.left}
                                onChange={v => {
                                    equations[i].left = v.target.value;
                                    setEquations([...equations]);
                                    // setVars(detectVars(flattenEqs()));
                                }}
                            />
                            <span> = </span>
                            <input
                                type="text"
                                className={`equation_input ${
                                    eq.err_right ? "err" : ""
                                }`}
                                defaultValue={eq.right}
                                onChange={v => {
                                    equations[i].right = v.target.value;
                                    setEquations([...equations]);
                                    // setVars(detectVars(flattenEqs()));
                                }}
                            />
                        </div>
                    ))}
                </div>
                <button className="solve_button" onClick={solve}>
                    Solve
                </button>
                {solution.length != 0 ? (
                    <>
                        <h4>Solutions:</h4>
                        {solution.map(([name, [re, im]], i) => (
                            <div key={i}>
                                <span className="solution">{`${name} = ${formatComplex(
                                    re,
                                    im
                                )}`}</span>
                                <br />
                            </div>
                        ))}
                    </>
                ) : (
                    <></>
                )}
            </div>
        </div>
    );
};

export default App;
