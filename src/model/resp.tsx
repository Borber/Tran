export interface Resp<T> {
    code: number
    msg: string
    data: T
}

export interface TransVO {
    word: boolean
    trans: Tran[]
    dicts: Dict[]
}

export interface Dict {
    pos: string
    terms: string[]
}

export interface Tran {
    typ: number
    data: string
}
