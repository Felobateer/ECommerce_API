<?php

namespace App\Controller;

#[Route('/login', name: 'app_login')]
 function login(AuthenticationUtils $authenticationUtils): Response {
    $error = $authenticationUtils->getLastAuthenticationError();
    $lastUsername = $authenticationUtils->getLastUsername();

    return $this->render('security/login.html.twig', [
        'last_username' => $lastUsername,
        'error' => $error,
    ]);
}
