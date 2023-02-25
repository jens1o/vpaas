import React from 'react';
import { Link } from 'react-router-dom';

export default class Menu extends React.Component {
    render() {
        return (
            <nav className='w-full bg-slate-300 rounded px-4'>
                <ul className='grid grid-rows-1 grid-cols-12 gap-4 py-5 text-center'>
                    <li>
                        <Link to="/">Home</Link>
                    </li>
                    <li>
                        <Link to="/blogs">Blogs</Link>
                    </li>
                    <li>
                        <Link to="/contact">Contact</Link>
                    </li>
                </ul>
            </nav>
        );
    }
}