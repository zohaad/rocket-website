window.addEventListener( "load", function () {
    let form = this.document.getElementById( 'loginForm' );
    
    async function postLoginForm() {
        var formData = new FormData( form );

        let res = await fetch( '/', {
            method: 'POST',
            body: formData,
            header: { 'Content-Type': 'application/x-www-form-urlencoded' },
        }).catch(err => console.error(err));

        if (res.ok) {
            console.log(await response.json());
        }
    }

    form.addEventListener( "submit", function( event ) {
        event.preventDefault();

        postLoginForm();
    });
});