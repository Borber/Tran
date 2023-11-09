const timestamp_number = () => {
    return new Date().getTime()
}

const timestamp_string = () => {
    return timestamp_number() + ""
}

export { timestamp_number, timestamp_string }
