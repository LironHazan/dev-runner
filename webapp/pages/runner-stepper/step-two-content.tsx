import * as React from 'react';
import StepContent from '@mui/material/StepContent';
import Button from '@mui/material/Button';
import {List, ListItem, ListItemText, TextField} from "@mui/material";
import styles from '../../styles/Stepper.module.css'
import {useEffect} from "react";
import {getCommands} from "../api/dev-runner-api";


export default function StepTwoContent() {

    const [commands, setCommands] = React.useState(['']);

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
