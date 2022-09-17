import {createBrowserRouter} from 'react-router-dom';
import App from './App';
import Runner from "./pages/runner/Runner";

export const router = createBrowserRouter([
    {
path: '',
    element: <App />,
    children: [
    {
        path: 'runner',
        element: <Runner/>,
    },
    {
        path: 'results-history',
        element: 'Results History'
    },
    {
        path: 'p3',
        element: `page 3 `,
    },
],
},
]);