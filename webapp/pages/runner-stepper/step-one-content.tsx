import * as React from 'react';
import StepContent from '@mui/material/StepContent';
import Button from '@mui/material/Button';
import {List, ListItem, ListItemText, TextField} from "@mui/material";
import styles from '../../styles/Stepper.module.css'


export default function StepOneContent() {

    const [path, setPath] = React.useState('');
    const [paths, setPaths] = React.useState(['']);

    const handleChange = (event: { target: { value: React.SetStateAction<string> } }) => setPath(event.target.value);

    const onEnterEvt = (event: { key: string; }) => event.key === 'Enter' && addPath();

    const addPath = () => {
        if (paths.length < 5 && !paths.includes(path)) {
            setPaths([...paths, path])
        }
    }

    const renderPaths = () =>
         paths.map((value, index)  =>
             (<ListItem key={index}>
                    <ListItemText primary={value}/>
                </ListItem>));


    return (
            <StepContent>
                <TextField
                    className={styles.textField}
                    value={path}
                    onKeyPress={onEnterEvt}
                    onChange={handleChange}
                    label="Enter full project path"
                    variant="standard" />
                <Button variant="text" onClick={addPath}>Add another path</Button>
                <List dense={true}>
                    {renderPaths()}
                </List>
            </StepContent>
    );
}
