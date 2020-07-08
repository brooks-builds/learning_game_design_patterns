<php

class LaserVision
{
    public function thisIsPublic()
    {
        $this->thisIsPrivate();
    }

    private function thisIsPrivate()
    {}

    public static function thisIsPublicAndStatic()
    {}

    protected function thisIsProtected()
    {}
}

class BlueLaserVision extends LaserVision
{
    public function thisIsPublic()
    {
        $this->thisIsProtected();
    }
}

LaserVision::thisIsPublicAndStatic();

$laserVision = new LaserVision;
$laserVision->thisIsPublic();