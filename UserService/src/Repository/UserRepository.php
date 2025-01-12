<?php

namespace App\Repository;

use App\Entity\User;
use Doctrine\Bundle\DoctrineBundle\Repository\ServiceEntityRepository;
use Doctrine\Persistence\ManagerRegistry;

class UserRepository extends ServiceEntityRepository
{
    public function __construct(ManagerRegistry $registry)
    {
        parent::__construct($registry, User::class);
    }

    // Find a user by their ID
    public function findById(int $id): ?User
    {
        return $this->find($id);
    }

    // Find a user by their email
    public function findByEmail(string $email): ?User
    {
        return $this->findOneBy(['email' => $email]);
    }

    // Get all users
    public function findAllUsers(): array
    {
        return $this->findAll();
    }

    // Update user details
    public function updateUser(User $user, array $data): User
    {
        if (isset($data['firstName'])) {
            $user->setFirstName($data['firstName']);
        }
        if (isset($data['lastName'])) {
            $user->setLastName($data['lastName']);
        }
        if (isset($data['email'])) {
            $user->setEmail($data['email']);
        }
        if (isset($data['address'])) {
            $user->setAddress($data['address']);
        }
        if (isset($data['phoneNumber'])) {
            $user->setPhoneNumber($data['phoneNumber']);
        }
        if (isset($data['dateOfBirth'])) {
            $user->setDateOfBirth(new \DateTime($data['dateOfBirth']));
        }

        $this->_em->persist($user);
        $this->_em->flush();

        return $user;
    }
}