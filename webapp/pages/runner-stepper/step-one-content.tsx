import * as React from 'react';
import StepContent from '@mui/material/StepContent';
import Button from '@mui/material/Button';
import {Alert, List, ListItem, ListItemText, TextField} from "@mui/material";
import styles from '../../styles/Stepper.module.css'
import {setRunnableProject} from "./dev-runner-api";

interface StepOneProps {
    enableContinue: (enableContinueBtn: boolean) => void;
}

export default function StepOneContent({enableContinue}: StepOneProps) {

    const [errMsg, setError] = React.useState('');
    const [path, setPath] = React.useState('');
    const [paths, setPaths] = React.useState(['']);

    const handleChange = (event: { target: { value: React.SetStateAction<string> } }) => {
        setError('');
        setPath(event.target.value);
    }

    const onEnterEvt = (event: { key: string; }) => event.key === 'Enter' && addPath();

    const addPath = async () => {
        if (paths.length < 5 && !paths.includes(path)) {
            try {
                const response = await setRunnableProject(path);
                // Won't push new path in case its invalid
                response && setPaths([...paths, path]);
                enableContinue(true);
            } catch (e) {
                setError('Invalid filepath!');
            }
        }
    }

    const renderPaths = () =>
         paths.map((value, index)  =>
             (<ListItem key={index}>
                    <ListItemText primary={value}/>
                </ListItem>));


    return (
            <StepContent className="stepOne">
                <TextField
                    className={styles.textField}
                    value={path}
                    onKeyPress={onEnterEvt}
                    onChange={handleChange}
                    label="Enter full project path"
                    variant="standard" />
                { errMsg !== '' ? <Alert className={styles.alertPos} severity="error">{errMsg}</Alert> : null }
                <Button className={styles.btnPos} variant="text" onClick={addPath}>Add path</Button>
                <List dense={true}>
                    {renderPaths()}
                </List>
            </StepContent>
    );
}
