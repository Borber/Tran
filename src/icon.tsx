interface IconProps {
    size?: number
}

export const Minimize = (props: IconProps) => {
    return (
        <svg
            aria-hidden="false"
            width={props.size}
            height={props.size}
            viewBox="0 0 12 12">
            <rect fill="currentColor" width="10" height="1" x="1" y="6" />
        </svg>
    )
}

export const Maximize = (props: IconProps) => {
    return (
        <svg
            aria-hidden="false"
            width={props.size}
            height={props.size}
            viewBox="0 0 12 12">
            <rect
                width="9"
                height="9"
                x="1.5"
                y="1.5"
                fill="none"
                stroke="currentColor"
            />
        </svg>
    )
}

export const Close = (props: IconProps) => {
    return (
        <svg
            aria-hidden="false"
            width={props.size}
            height={props.size}
            viewBox="0 0 12 12">
            <polygon
                fill="currentColor"
                fill-rule="evenodd"
                points="11 1.576 6.583 6 11 10.424 10.424 11 6 6.583 1.576 11 1 10.424 5.417 6 1 1.576 1.576 1 6 5.417 10.424 1"
            />
        </svg>
    )
}

export const PinIcon = (props: IconProps) => {
    return (
        <svg
            data-icon="pin"
            width={props.size}
            height={props.size}
            role="img"
            viewBox="0 0 16 16">
            <path
                d="M9.41.92c-.51.51-.41 1.5.15 2.56L4.34 7.54C2.8 6.48 1.45 6.05.92 6.58l3.54 3.54-3.54 4.95 4.95-3.54 3.54 3.54c.53-.53.1-1.88-.96-3.42l4.06-5.22c1.06.56 2.04.66 2.55.15L9.41.92z"
                fill-rule="evenodd"
            />
        </svg>
    )
}

export const CopyIcon = (props: IconProps) => {
    return (
        <svg
            data-icon="duplicate"
            width={props.size}
            height={props.size}
            role="img"
            viewBox="0 0 16 16">
            <path
                d="M15 0H5c-.55 0-1 .45-1 1v2h2V2h8v7h-1v2h2c.55 0 1-.45 1-1V1c0-.55-.45-1-1-1zm-4 4H1c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h10c.55 0 1-.45 1-1V5c0-.55-.45-1-1-1zm-1 10H2V6h8v8z"
                fill-rule="evenodd"
            />
        </svg>
    )
}

export const HideIcon = (props: IconProps) => {
    return (
        <svg
            data-icon="cross"
            width={props.size}
            height={props.size}
            role="img"
            viewBox="0 0 16 16">
            <path
                d="M9.41 8l3.29-3.29c.19-.18.3-.43.3-.71a1.003 1.003 0 00-1.71-.71L8 6.59l-3.29-3.3a1.003 1.003 0 00-1.42 1.42L6.59 8 3.3 11.29c-.19.18-.3.43-.3.71a1.003 1.003 0 001.71.71L8 9.41l3.29 3.29c.18.19.43.3.71.3a1.003 1.003 0 00.71-1.71L9.41 8z"
                fill-rule="evenodd"
            />
        </svg>
    )
}
