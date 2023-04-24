import { useState } from "react";
import "./App.css";

import * as wasm from "../wasm-lib/pkg/wasm_lib";
import { DEFAULT_EQS, detectVars } from "./equation";
import { PositionedError } from "../wasm-lib/pkg/wasm_lib";

const DESCRIPTIONS = [
    {
        text: [
            "This is a numerical solver for systems of equations.",
            "It works by using a version of the Newton-Raphson method\
            modified to work with matrices. An initial random guess for each\
            variable is picked, after which this iterative method is applied\
            until convergence.",
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
    const flattenEqs = () => equations.flatMap(([_, a, b]) => [a, b]);

    let [vars, setVars] = useState(detectVars(flattenEqs()));

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
                <div className="solver">
                    <span>
                        <button
                            className="add_button"
                            onClick={() => {
                                setEquations([
                                    ...equations,
                                    [Math.random(), "", ""],
                                ]);
                            }}
                        >
                            Add equation
                        </button>
                        Detected variables: {vars.join(", ")}
                    </span>
                    <br />

                    {equations.map(([id, left, right], i) => (
                        <div className="equation" key={id}>
                            <button
                                className="remove_button"
                                onClick={() => {
                                    equations.splice(i, 1);
                                    setEquations([...equations]);
                                }}
                            >
                                <span className="material-symbols-outlined">
                                    delete
                                </span>
                            </button>
                            <input
                                type="text"
                                className="equation_input"
                                defaultValue={left}
                                onChange={v => {
                                    equations[i][1] = v.target.value;
                                    setEquations([...equations]);
                                    setVars(detectVars(flattenEqs()));
                                }}
                            />
                            <span> = </span>
                            <input
                                type="text"
                                className="equation_input"
                                defaultValue={right}
                                onChange={v => {
                                    equations[i][1] = v.target.value;
                                    setEquations([...equations]);
                                    setVars(detectVars(flattenEqs()));
                                }}
                            />
                        </div>
                    ))}
                </div>
                <button
                    onClick={() => {
                        try {
                            console.log(
                                wasm.solve(equations.map(([_, a, b]) => [a, b]))
                            );
                        } catch (e) {
                            if (e instanceof PositionedError) {
                                console.log(e.msg);
                                console.log(e.eq);
                                console.log(e.second);
                            }
                        }
                        // wasm.greet(equations[0][1]);
                    }}
                >
                    test
                </button>
            </div>
        </div>
    );
};

export default App;
