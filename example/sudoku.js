import * as Solver from "../pkg/sudoku.js";
import init from "../pkg/sudoku.js";

const fieldEasy = [
    9, 4, 5, -1, -1, 8, -1, -1, 6,
    2, -1, 3, -1, 6, -1, -1, -1, 5,
    -1, -1, -1, 5, 4, 7, -1, 3, 2,

    7, -1, -1, -1, -1, 3, 2, 6, 9,
    3, -1, 4, -1, -1, 2, -1, -1, -1,
    -1, -1, 6, -1, 1, 9, 8, 4, -1,

    -1, -1, -1, 8, -1, -1, 5, 7, 1,
    6, 8, -1, -1, -1, -1, -1, -1, -1,
    -1, 5, -1, 3, 2, -1, -1, -1, 8,
];

const fieldMedium = [
    6, 5, -1, 7, 9, -1, 8, -1, -1,
    -1, -1, -1, -1, -1, 6, 3, -1, -1,
    4, 1, -1, -1, -1, -1, 5, -1, -1,

    -1, -1, 6, -1, -1, -1, -1, 1, -1,
    -1, 3, -1, -1, 8, 1, -1, -1, -1,
    -1, 2, 1, -1, 3, -1, -1, 6, -1,

    3, -1, -1, -1, -1, -1, -1, -1, 7,
    8, -1, -1, -1, -1, -1, -1, -1, 4,
    -1, -1, -1, 8, 5, 9, -1, 3, -1,
];

const fieldHard = [
    -1, -1, -1, 9, 6, -1, 1, 3, -1,
    -1, -1, -1, 2, 1, -1, -1, -1, 4,
    -1, 8, 4, -1, -1, -1, -1, 2, -1,

    -1, -1, -1, -1, -1, 3, -1, -1, -1,
    9, -1, 6, 7, -1, -1, 8, -1, 2,
    2, -1, -1, -1, -1, 1, -1, -1, 6,

    3, 7, 8, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, 1, 7, 6, -1, -1, -1,
    -1, -1, -1, -1, -1, -1, 9, 4, -1,
];

const fieldExtraHard = [
    7, -1, -1, -1, -1, 4, 8, -1, -1,
    -1, -1, -1, -1, -1, 5, 4, -1, -1,
    -1, -1, 9, -1, -1, -1, 7, -1, -1,

    4, -1, -1, -1, -1, -1, -1, 9, -1,
    8, -1, 7, -1, -1, -1, -1, -1, -1,
    -1, -1, -1, 6, 1, -1, -1, -1, -1,

    -1, 3, -1, -1, 5, -1, -1, -1, 1,
    -1, 1, -1, 2, -1, -1, -1, 7, 5,
    -1, -1, -1, 1, 4, 3, -1, -1, -1,
];

const {createApp} = Vue;

const app = createApp({
    // language=HTML
    template: `
        <div class="container">
            <div v-for="n in 9" class="row" :class="'n-' + n">
                <div v-for="m in 9" class="col" :class="'n-' + m">
                    <div class="field" v-if="field[(n-1) * 9 + (m-1)] !== -1">{{field[(n-1) * 9 + (m-1)]}}</div>
                    <div v-else class="solution" v-if="isSolved && solution[(n-1) * 9 + (m-1)] !== -1">
                        {{solution[(n-1) * 9 + (m-1)]}}
                    </div>
                </div>
            </div>
            <button @click="solve">Solve</button><br>
            <select v-model="fieldIndex" @change="solution = null">
                <option v-for="i in 4" :value="i-1">{{i}}</option>
            </select>
        </div>
    `,
    data() {
        return {
            fields: [
                fieldEasy,
                fieldMedium,
                fieldHard,
                fieldExtraHard
            ],
            fieldIndex: 0,
            solution: null,
        }
    },
    computed: {
        isSolved() {
            return this.solution !== null
        },
        field() {
            return this.fields[this.fieldIndex]
        }
    },
    methods: {
        solve() {
            this.solution = Solver.solve(Int8Array.from(this.field))
        }
    }
});

(async () => {
    await init();

    app.mount('#app');
})();

