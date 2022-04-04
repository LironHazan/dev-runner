import { useState } from 'react';
import StepContent from '@mui/material/StepContent';
import {List, ListItem, ListItemText, TextField} from "@mui/material";
import {useEffect} from "react";
import {getCommands} from "./dev-runner-api";


export default function StepTwoContent() {

    const [commands, setCommands] = useState(['']);

    useEffect(() => {
        getCommands().then((res: {scripts: string[]}) => {
            setCommands(res?.scripts)
        });
    }, [])

    const renderCommands = () =>
        commands?.map((value, index)  =>
             (<ListItem key={index}>
                    <ListItemText primary={value}/>
                </ListItem>));


    return (
            <StepContent>
                <List dense={true}>
                    {renderCommands()}
                </List>
            </StepContent>
    );
}
