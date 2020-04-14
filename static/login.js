window.addEventListener( "load", function () {
    function sendData() {
        const XHR = new XMLHttpRequest();

        const FD = new FormData( loginForm );

        // ???
        XHR.addEventListener( "load", function( event ) {
            // I think I need to put the "response handling logic" here
        });

        // ???
        XHR.addEventListener( "error", function( event ) {
            alert( 'Something went wrong' );
        });

        XHR.open( "POST", "/" );

        XHR.send( FD )

    }

    let loginForm = this.document.getElementById( "loginForm" );

    loginForm.addEventListener( "submit", function( event ) {
        event.preventDefault();

        sendData();
    });
});