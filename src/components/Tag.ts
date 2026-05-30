export class Tag {
    public name: string;
    public bind_count: number;
    public structure: string;

    constructor(name: string, bind_count: number, structure: string,) {
        this.name = name;
        this.bind_count = bind_count;
        this.structure = structure;
    }
}