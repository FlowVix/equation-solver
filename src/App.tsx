import { useState } from "react";
import "./App.css";

import * as wasm from "../wasm-lib/pkg/wasm_lib";

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
            "Predefined constants: E, PI, I",
            "Complex numbers are supported",
        ],
    },
];

const DEFAULT_EQS: [number, string, string][] = [
    [Math.random(), "a * (b + 4)", "14"],
    [Math.random(), "a + b", "5"],
];

const App = () => {
    let [equations, setEquations] = useState([...DEFAULT_EQS]);

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
                    <button
                        className="add_button"
                        onClick={() => {
                            wasm.test(6);
                            setEquations([
                                ...equations,
                                [Math.random(), "", ""],
                            ]);
                        }}
                    >
                        Add equation
                    </button>
                    <br />

                    {equations.map(([id, left, right], i) => (
                        <div className="equation" key={id}>
                            <button
                                className="remove_button"
                                onClick={() => {
                                    console.log(equations);
                                    equations.splice(i, 1);
                                    console.log(equations);
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
                                }}
                            />
                        </div>
                    ))}
                </div>
            </div>
        </div>
    );
};

export default App;
