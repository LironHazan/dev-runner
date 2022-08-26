import {List, ListItemButton, ListItemText} from "@mui/material";
import { invoke } from "@tauri-apps/api/tauri";

export default function StepTwoContent({commands}: {commands: string[]}) {

    const handleListItemClick = async (_: any, command: string) => {
        const response = await invoke("exec_command", { command });
        console.log(response);
    };

    const renderCommands = () =>
        commands?.map((value, index)  =>
             (<ListItemButton key={crypto.randomUUID()}
                              onClick={(event) => handleListItemClick(event, value)}>
                    <ListItemText primary={value}/>
                </ListItemButton>));


    return (
            <>
                <List>
                    {renderCommands()}
                </List>
            </>
    );
}
