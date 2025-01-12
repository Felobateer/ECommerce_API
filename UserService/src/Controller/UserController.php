<?php

namespace App\Controller;

use App\Repository\UserRepository;
use Symfony\Component\HttpFoundation\JsonResponse;
use Symfony\Component\HttpFoundation\Request;
use Symfony\Component\HttpFoundation\Response;
use Symfony\Component\Routing\Annotation\Route;
use Symfony\Component\Security\Core\Encoder\UserPasswordEncoderInterface;
use Symfony\Component\Serializer\SerializerInterface;

class UserController extends AbstractController
{
    private UserRepository $userRepository;

    public function __construct(
        UserRepository $userRepository,
        UserPasswordEncoderInterface $passwordEncoder,
        SerializerInterface $serializer)
    {
        $this->userRepository = $userRepository;
        $this->passwordEncoder = $passwordEncoder;
        $this->serializer = $serializer;
    }

    /**
     * @Route("/user/{id}", name="get_user_by_id", methods={"GET"})
     */
    public function getUserById(int $id): JsonResponse
    {
        $user = $this->userRepository->findById($id);

        if (!$user) {
            return new JsonResponse(['message' => 'User not found'], 404);
        }

        return $this->json($user);
    }

    /**
     * @Route("/user/email/{email}", name="get_user_by_email", methods={"GET"})
     */
    public function getUserByEmail(string $email): JsonResponse
    {
        $user = $this->userRepository->findByEmail($email);

        if (!$user) {
            return new JsonResponse(['message' => 'User not found'], 404);
        }

        return $this->json($user);
    }

    /**
     * @Route("/users", name="get_all_users", methods={"GET"})
     */
    public function getAllUsers(): JsonResponse
    {
        $users = $this->userRepository->findAllUsers();

        return $this->json($users);
    }

    /**
     * @Route("/user/{id}/edit", name="edit_user", methods={"PUT"})
     */
    public function editUser(int $id, Request $request): JsonResponse
    {
        $user = $this->userRepository->findById($id);

        if (!$user) {
            return new JsonResponse(['message' => 'User not found'], 404);
        }

        $data = json_decode($request->getContent(), true);

        $updatedUser = $this->userRepository->updateUser($user, $data);

        return $this->json($updatedUser);
    }

    /**
     * @Route("/user/{id}/delete", name="delete_user", methods={"DELETE"})
     */

    public function deleteUser(int $id): JsonResponse
    {
        $user = $this->userRepository->find($id);

        if (!$user) {
            return new JsonResponse(['error' => 'User not found'], Response::HTTP_NOT_FOUND);
        }

        $this->entityManager->remove($user);
        $this->entityManager->flush();

        return new JsonResponse(['message' => 'User deleted successfully'], Response::HTTP_OK);
    }

    /**
     * @Route("/authenticate", name="user_authenticate", methods={"POST"})
     */
    public function authenticate(Request $request): JsonResponse
    {
        $content = json_decode($request->getContent(), true);

        if (!isset($content['email']) || !isset($content['password'])) {
            return new JsonResponse(
                ['error' => 'Email and password are required'],
                Response::HTTP_BAD_REQUEST
            );
        }

        $user = $this->userRepository->findOneBy(['email' => $content['email']]);

        if (!$user || !$this->passwordEncoder->isPasswordValid($user, $content['password'])) {
            return new JsonResponse(
                ['error' => 'Invalid credentials'],
                Response::HTTP_UNAUTHORIZED
            );
        }

        // You can include additional data here, such as roles or tokens.
        $responseData = [
            'id' => $user->getId(),
            'email' => $user->getEmail(),
            'roles' => $user->getRoles(),
        ];

        return new JsonResponse($responseData, Response::HTTP_OK);
    }
}