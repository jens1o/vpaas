import React from 'react';
import ReactDOM from 'react-dom/client';
import './index.css';
import Menu from './menu';
import { Routes, Route, createBrowserRouter, RouterProvider } from "react-router-dom";
import MainRoute from './routes/mainRoute';

class VPAAS extends React.Component {
    render() {
        return (
            <div className='container mx-auto'>
                <Menu />
                <Routes>
                    <Route path="" element={<MainRoute />} />
                </Routes>
            </div>
        );
    }
}

const router = createBrowserRouter([
    {
        path: "/",
        element: <VPAAS />,
    },
]);

// ========================================

const root = ReactDOM.createRoot(document.getElementById("root"));
root.render(<React.StrictMode>
    <RouterProvider router={router} />
</React.StrictMode>);
