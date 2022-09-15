import * as React from 'react';
import {Alert, Box, List, ListItem, ListItemText} from "@mui/material";
import {invoke} from "@tauri-apps/api";


export default function StepOneContent({liftCommands}: { liftCommands: (cmds: string[]) => void }) {

    const [errMsg, setError] = React.useState('');
    const [path, setPath] = React.useState('');
    const [paths, setPaths] = React.useState([] as string[]);
    const [commands, setCommands] = React.useState([] as string[]);

    const handleChange = (event: { target: { value: React.SetStateAction<string> } }) => {
        setError('');
        setPath(event.target.value);
    }

    const onEnterEvt = (event: { key: string; }) => event.key === 'Enter' && addPath();

    const addPath = async () => {
        if (paths.length < 5 && !paths.includes(path)) {
                const response = await invoke("set_runnable_project", { path });
                // Won't push new path in case its invalid
                response === "OK" ? setPaths([...paths, path]) : setError('Invalid filepath!');
        }
    }

    const getCommands = async () => {
        const response: string[] = await invoke("get_commands", { path });
        liftCommands(response);
        setCommands(response);
    }

    const renderPaths = () =>
         paths.map((value, index)  =>
             (<ListItem key={index}>
                    <ListItemText primary={value}/>
                </ListItem>));


    return (
            <>
                <Box sx={{ display: 'flex', gap: 2, justifyContent: 'center' }}>
                    <input
                        onChange={handleChange}
                        placeholder="Enter a full project path..."
                    />
                    { errMsg !== '' ? <Alert className="alertPos" severity="error">{errMsg}</Alert> : null }
                    <button type="button" onClick={addPath}>Add path</button>
                </Box>
                <List dense={true}>
                    {renderPaths()}
                </List>
                { paths.length !== 0 && commands.length === 0 && <button type="button" onClick={getCommands}>Get Commands</button> }
            </>
    );
}
