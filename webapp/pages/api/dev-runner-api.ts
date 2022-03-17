
export function setRunnableProjects(paths: string[]): Promise<void> {
    return fetch('http://localhost:3333/list', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ paths })
    }).then(data => data.json())
}

export function getCommands(): Promise<string[]> {
    return fetch('http://localhost:3333/list')
        .then(data => data.json())
}