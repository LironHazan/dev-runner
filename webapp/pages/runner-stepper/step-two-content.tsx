import * as React from 'react';
import StepContent from '@mui/material/StepContent';
import Button from '@mui/material/Button';
import {List, ListItem, ListItemText, TextField} from "@mui/material";
import styles from '../../styles/Stepper.module.css'
import {useEffect} from "react";


export default function StepTwoContent() {

    const [commands, setCommands] = React.useState(['']);

    useEffect(() => {
        updateCommands();
    }, [])

    const updateCommands = () => {
        setCommands(['start', 'build', 'publish'])
    }

    const renderCommands = () =>
        commands.map((value, index)  =>
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
