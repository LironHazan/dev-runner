import { SetStateAction, useState} from 'react';
import StepContent from '@mui/material/StepContent';
import {List, ListItem, ListItemButton, ListItemText, TextField} from "@mui/material";
import {useEffect} from "react";
import {getCommands} from "./dev-runner-api";


export default function StepTwoContent({selectedCommand: selectedCommand}: any) {

    const [commands, setCommands] = useState(['']);
    const [selectedIndex, setSelectedIndex] = useState(-1);

    const handleListItemClick = (event: MouseEvent, index: SetStateAction<number>, command: string) => {
        setSelectedIndex(index);
        selectedCommand(command)
    };

    useEffect(() => {
        getCommands().then((res: {scripts: string[]}) => {
            setCommands(res?.scripts)
        });
    }, [])

    const renderCommands = () =>
        commands?.map((value, index)  =>
             (<ListItemButton key={index} // usually its not recommended to use index when an item could be dropped
                              selected={selectedIndex === index}
                              onClick={(event) => handleListItemClick((event as any), index, value)}>
                    <ListItemText primary={value}/>
                </ListItemButton>));


    return (
            <StepContent>
                <List dense={true}>
                    {renderCommands()}
                </List>
            </StepContent>
    );
}
