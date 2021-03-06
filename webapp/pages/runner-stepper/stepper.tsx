import Box from '@mui/material/Box';
import Stepper from '@mui/material/Stepper';
import Step from '@mui/material/Step';
import StepLabel from '@mui/material/StepLabel';
import StepContent from '@mui/material/StepContent';
import Button from '@mui/material/Button';
import Paper from '@mui/material/Paper';
import Typography from '@mui/material/Typography';
import StepOneContent from "./step-one-content";
import StepTwoContent from "./step-two-content";
import {execScript} from "./dev-runner-api";
import styles from '../../styles/Stepper.module.css'
import {useState} from "react";

const steps = [
    {
        label: 'Set runnable projects',
    },
    {
        label: 'Select a command to execute',
    }
    ];

export default function VerticalLinearStepper() {
    const [activeStep, setActiveStep] = useState(0);
    const [activeContinueBtn, setContinue] = useState(false);
    const [selectedCommand, setSelectedCommand] = useState('');

    const getSelectedCommand = (command: string) => {
        setSelectedCommand(command)
    }

    const handleNext = () => {
        setActiveStep((prevActiveStep) => prevActiveStep + 1);
        if (activeStep === steps.length - 1) {
            execScript(selectedCommand).then((done) => {
                console.log(done);
            })
        }
    };

    const handleBack = () => {
        setActiveStep((prevActiveStep) => prevActiveStep - 1);
        // todo: better to move this
        setContinue(false);
    };

    const handleReset = () => {
        setActiveStep(0);
    };

    return (
        <Box sx={{ maxWidth: 400 }}>
            <Stepper activeStep={activeStep} orientation="vertical" className="stepper">
                {steps.map((step, index) => (
                    <Step key={step.label}>
                        <StepLabel
                            optional={
                                index === 2 ? (
                                    <Typography variant="caption">Last step</Typography>
                                ) : null
                            }
                        >
                            {step.label}
                        </StepLabel>
                        <StepContent>
                            { index === 0 ? (<StepOneContent enableContinue={(enableContinueBtn: boolean) => {
                                setContinue(enableContinueBtn);
                            }
                            }/>) : null }
                            { index === 1 ? (<StepTwoContent selectedCommand={getSelectedCommand}/>) : null }
                            <Box sx={{ mb: 2 }}>
                                <div className={styles.nextButtonsG}>
                                    <Button
                                        disabled={!activeContinueBtn}
                                        variant="contained"
                                        onClick={handleNext}
                                        sx={{ mt: 1, mr: 1 }}
                                    >
                                        {index === steps.length - 1 ? 'Run!' : 'Continue'}
                                    </Button>
                                    <Button
                                        disabled={index === 0}
                                        onClick={handleBack}
                                        sx={{ mt: 1, mr: 1 }}
                                    >
                                        Back
                                    </Button>
                                </div>
                            </Box>
                        </StepContent>
                    </Step>
                ))}
            </Stepper>
            {activeStep === steps.length && (
                <Paper square elevation={0} sx={{ p: 3 }}>
                    <Typography>All steps completed</Typography>
                    <Button onClick={handleReset} sx={{ mt: 1, mr: 1 }}>
                        Reset
                    </Button>
                </Paper>
            )}
        </Box>
    );
}
