
async function basicRequest<T>(url: string, method: 'POST' | 'GET' | 'DELETE' | 'PUT', payload?: Record<string, T>): Promise<any> {
    const data = await fetch(url, {
        method: method,
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify(payload)
    });
    return data.json();
}

export function setRunnableProject(path: string): Promise<{msg: string}> {
    return basicRequest('http://localhost:8080/set-runnable-project', 'POST', { path });
}

export function execScript(script: string): Promise<{msg: string}> {
    return basicRequest('http://localhost:8080/exec-command', 'POST', { script });
}

export function getCommands(): Promise<{scripts: string[]}> {
    return basicRequest('http://localhost:8080/get-commands', 'GET');
}