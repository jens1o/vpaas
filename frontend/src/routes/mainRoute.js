import React from 'react';
import Heading from '../components/Heading';

export default class MainRoute extends React.Component {

    constructor(props) {
        super(props);
        this.state = { fileStream: null };
        this.fileRef = React.createRef();
    }

    render() {
        return (
            <main className='text-center mt-5 py-5 bg-gray-100 rounded-xl'>
                <Heading>Upload your video</Heading>

                <section className='bg-gray-200 max-w-lg mx-auto mt-5 p-5 rounded-lg'>
                    {(this.state.fileStream === null) ? (<form onSubmit={e => this.onFormSubmit(e)} encType="multipart/form-data">
                        <input type="file" ref={this.fileRef} className='inline-block' accept='video/*' />

                        <input type="submit" className='block bg-blue-400 p-5 m-5 mx-auto rounded-md text-gray-100' value={"Downscale to 240p"} />
                    </form>) : (<div>uploaded</div>)}
                </section>
            </main>
        );
    }

    onFormSubmit(e) {
        const file = this.fileRef.current.files[0];
        const formData = new FormData();

        formData.append('file', file);
        formData.append('new_dimension', JSON.stringify({ width: 320, height: 240 }));

        e.preventDefault();

        fetch('http://127.0.0.1:9000/videos', {
            // content-type header should not be specified!
            method: 'POST',
            body: formData,
        })
            .then(response => response.json())
            .then(success => {
                // Do something with the successful response
            })
            .catch(error => console.log(error)
            );

        return false;
    }
}