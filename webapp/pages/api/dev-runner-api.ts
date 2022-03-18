
export function setRunnableProject(path: string): Promise<{msg: string}> {
    console.log({ path })
    return fetch('http://localhost:8080/set-runnable-project', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ path })
    }).then(data => data?.json())
}

export function getCommands(): Promise<string[]> {
    return fetch('http://localhost:8080/list')
        .then(data => data.json())
}