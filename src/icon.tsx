interface IconProps {
    size?: number
}

export const Minimize = (props: IconProps) => {
    return (
        <svg
            aria-hidden="false"
            width={props.size}
            height={props.size}
            viewBox="0 0 12 12"
        >
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
            viewBox="0 0 12 12"
        >
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
            viewBox="0 0 12 12"
        >
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
            viewBox="0 0 16 16"
        >
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
            viewBox="0 0 16 16"
        >
            <path
                d="M15 0H5c-.55 0-1 .45-1 1v2h2V2h8v7h-1v2h2c.55 0 1-.45 1-1V1c0-.55-.45-1-1-1zm-4 4H1c-.55 0-1 .45-1 1v10c0 .55.45 1 1 1h10c.55 0 1-.45 1-1V5c0-.55-.45-1-1-1zm-1 10H2V6h8v8z"
                fill-rule="evenodd"
            />
        </svg>
    )
}

export const GithubIcon = (props: IconProps) => {
    return (
        <svg
            data-icon="duplicate"
            width={props.size}
            height={props.size}
            role="img"
            viewBox="0 0 100 100"
            xmlns="http://www.w3.org/2000/svg"
        >
            <circle cx="52" cy="52" r="44" opacity=".35" />
            <circle cx="50" cy="50" r="44" fill="#f2f2f2" />
            <path
                fill="#707cc0"
                d="M50,12.5c-20.711,0-37.5,16.789-37.5,37.5S29.289,87.5,50,87.5S87.5,70.711,87.5,50 S70.711,12.5,50,12.5z"
            />
            <path
                fill="#f2f2f2"
                d="M60.161,83.936c0-1.122,0.042-4.813,0.042-9.389c0-3.192-1.095-5.281-2.324-6.338 c7.624-0.847,15.626-3.74,15.626-16.888c0-3.736-1.324-6.791-3.518-9.184c0.352-0.866,1.527-4.346-0.341-9.057 c0,0-2.868-0.92-9.402,3.508c-2.734-0.759-5.662-1.139-8.568-1.152c-2.91,0.013-5.838,0.393-8.568,1.152 c-6.538-4.429-9.411-3.508-9.411-3.508c-1.862,4.712-0.687,8.192-0.336,9.057c-2.189,2.393-3.523,5.448-3.523,9.184 c0,13.115,7.99,16.051,15.589,16.915c-0.978,0.856-1.862,2.364-2.173,4.575c-1.95,0.876-6.907,2.386-9.96-2.844 c0,0-1.808-3.285-5.242-3.527c0,0-3.342-0.043-0.235,2.08c0,0,2.244,1.053,3.8,5.006c0,0,2.009,6.656,11.529,4.588 c0.017,2.856,0.046,5.008,0.046,5.821c0,0.385-0.122,0.792-0.383,1.115C45.945,86.292,49.3,87,52.807,87 c2.981,0,5.853-0.509,8.576-1.417C60.502,85.37,60.161,84.613,60.161,83.936z"
            />
            <path
                fill="#40396e"
                d="M50,89c-21.505,0-39-17.495-39-39s17.495-39,39-39s39,17.495,39,39S71.505,89,50,89z M50,14 c-19.851,0-36,16.149-36,36s16.149,36,36,36s36-16.149,36-36S69.851,14,50,14z"
            />
        </svg>
    )
}
